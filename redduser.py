#!/usr/bin/env python3
"""
@summary: download all my reddit comments - from now, back 'days' into the past           
@since:   26/March/2019
@author:  https://github.com/drandreaskrueger
@see:     https://github.com/drandreaskrueger/redduser for updates
"""

VERSION = "v0.1"

################################################################################
## Dependencies and defaults

import os
import time
import requests
import json
import argparse
from pprint import pprint

DEFAULTS = {"days"       : 2,  # how many days into the past?
            "filename"   : "comments.json", # output file
            "user"       : "andreaskrueger", # me, I have written this for myself, lol
            "bodyhead"   : 40, # print first x character of each post in statusline (0 for None)
            "sr_name_len": 12, # statusline: cut subreddit name after x characters  
            "limit"      : 10 } # comments per pagination - low for testing, high for production, max 100
assert DEFAULTS["limit"]<=100

HEADERS = {'User-agent': 'redduser v0.1'} # otherwise <Response Error: 429> 
SHORTLINK_COMMENT = "reddit.com/comments/{thread_id}/-/{comment_id}"
# SHORTLINK_COMMENT = "redd.it/{thread_id}/{comment_id}" # ???
API_URL="https://www.reddit.com/user/{user}/comments/.json?limit={limit}&after={after}"

################################################################################
## Code


def cli_args():
    """
    python3 redduser.py --help 
    """
    parser = argparse.ArgumentParser(description='Download recent reddit-comments of a user. Version %s.' % VERSION,
                                     formatter_class=argparse.ArgumentDefaultsHelpFormatter)
    parser.add_argument('--user', help='reddit username', default=DEFAULTS["user"],)
    parser.add_argument('--days', type=int, default=DEFAULTS["days"], help='go back how many days?')
    parser.add_argument('--file', help='output file', default=DEFAULTS["filename"])
    args = parser.parse_args()
    print( ("Arguments given: %s" % args).replace("Namespace", ""))
    return args


def utc(epoch):
    """
    epochtime --> human readable UTC time
    """
    return time.strftime("%Y%m%d-%H%M%S", time.gmtime(epoch))
# print (utc(epoch=1553604945.0)); exit()


def print_comments(children):
    """
    given a reddit response, process the data:
    print one child element at a time in a statusline
    returns 
    * the oldest appearing utc timestamp in this batch
    """
    datetimes=[] 
    for c in children:
        data=c["data"]
        sl_params={"comment_id" : data["id"],
                   "thread_id": data["link_id"][3:]} # get rid of the "t3_" prefix
        shortlink=SHORTLINK_COMMENT.format(**sl_params)
        body = data["body"] .replace('\r', ' ').replace('\n', ' ')
        linedata = (utc(data["created_utc"]), data["score"], 
                    data['subreddit_name_prefixed'][:DEFAULTS["sr_name_len"]], 
                    shortlink, body[:DEFAULTS["bodyhead"]])
        line = ("%s %+3d %-"+("%d"%DEFAULTS["sr_name_len"])+"s %s  %s") % linedata 
        print (line)
        datetimes.append(utc(data["created_utc"]))
    oldest = min(datetimes) 
    return oldest


def iterate(params, days):
    """
    Collates all comments from now, back to 'days' ago;
    infinite loop is only exited when an older timestamp is seen.
    Loop repeatedly calls reddit API with each time an older 'after' parameter.
    
    Returns 
    * would be next 'after' parameter for next batch (even further into the past)
    * oldest timestamp contained
    * all comments in one long list
    """
    comments = []
    epochCutoff = time.time() - days * 24 * 60 * 60
    dateCutoff = utc(epochCutoff)
    print ("Downloading until I see a post which was before '%s' UTC." % dateCutoff)
    oldest = None
    
    while True:
        url=API_URL.format(**params)
        print ("\n" + url, end=" ")
        
        try:
            res = requests.get(url, headers=HEADERS)
            r = res.json() # pprint(r)
        except Exception as e:
            print ("\nERROR: %s.%s\n %s" % (type(e).__module__, type(e).__name__, e))
            exit()
        print(res)
        
        # after = r["data"]["after"]
        # if params["after"] == after:
            # print ("Returned pagination parameter 'after' has not changed. No more posts? Breaking from the loop without even processing that last result.")
            # break
        
        children=r["data"]["children"]
        oldest = print_comments(children)
        comments.extend(children)

        after = r["data"]["after"]
        if after == None:
            print ("Returned pagination parameter 'after' was 'None' = No more posts! Breaking from the loop.")
            break
        params["after"] = after
        
        if oldest < dateCutoff:
            print ("Oldest seen post was at '%s' which is more than %d days ago. Breaking from the loop." % (oldest, days))
            break
    
    return after, oldest, comments


def save_all(comments, filename):
    """
    writes list of all comments as JSON file
    """
    with open(filename, "w") as f:
        json.dump(comments, f)
    num=len(comments)
    size = os.path.getsize(filename)
    print("\nDone. Written %d comments to file '%s' (%d bytes)." % (num, filename, size))


if __name__ == '__main__':
    
    args = cli_args()
    
    params={"user":args.user, 
            "limit": DEFAULTS["limit"], 
            "after": "None"}
    _,_,comments = iterate(params, args.days)
    
    save_all(comments, args.file)
    