#!/usr/bin/env python3

"""
@summary: testing the reddit api first in python just because rust is stupid to use with json
@since:   13/April/2019
@author:  https://github.com/drandreaskrueger
@see:     https://github.com/drandreaskrueger/redduser for updates
"""

VERSION = "v0.1"

################################################################################
## Dependencies and defaults

from pprint import pprint
import argparse
import json
import requests
import time
import os

HEADERS = {'User-agent': 'redduser v0.1'}  # otherwise <Response Error: 429>

def understandCommentsFormat(url):
    print (url)
    r=requests.get(url, headers=HEADERS)
    j=r.json()
    print ("type(j)=%s | len(j)=%d" % (type(j).__name__,len(j)))

    print()
    print ("Element 0, keys:", j[0].keys())
    print ("Element 0, kind:", j[0]["kind"])
    print ("Element 0, data, keys:", j[0]["data"].keys())
    print ("Element 0, data, children --> len():", len(j[0]["data"]["children"]) )
    print ("Element 0, data, children, 0, keys:", j[0]["data"]["children"][0].keys() )
    print ("Element 0, data, children, 0, kind:", j[0]["data"]["children"][0]["kind"] )
    print ("Element 0, data, children, 0, data, keys:", sorted(j[0]["data"]["children"][0]["data"].keys()) )
    print ("Element 0, data, children, 0, data ...")
    data = j[0]["data"]["children"][0]["data"]
    print ("name & id: %s %s" % (data["name"], data["id"]))
    print ("author: %s" % (data["author"]))
    print ("title: %s" % (data["title"]))
    print ("selftext: %s" % (data["selftext"]))

    print()
    print ("Element 1, keys:", j[1].keys())
    print ("Element 1, kind:", j[1]["kind"])
    print ("Element 1, data, keys:", j[1]["data"].keys())
    print ("Element 1, data, children --> len():", len(j[1]["data"]["children"]) )
    if 0 == len(j[1]["data"]["children"]):
        print ("Comment is MISSING")
    else:
        print ("Element 1, data, children, 0, keys:", j[1]["data"]["children"][0].keys() )
        print ("Element 1, data, children, 0, kind:", j[1]["data"]["children"][0]["kind"] )
        print ("Element 1, data, children, 0, data, keys:", sorted(j[1]["data"]["children"][0]["data"].keys()) )
        print ("Element 1, data, children, 0, data ...")
        data = j[1]["data"]["children"][0]["data"]
        print ("name & id: %s %s" % (data["name"], data["id"]))
        print ("author: %s" % (data["author"]))
        print ("body: %s" % (data["body"]))
    print (50*"-" +"\n")

def checkManyComments(jsonfile="comments.json"):
    with open(jsonfile) as f:
        j = json.load(f)
    for e in j:
        url = "https://www.reddit.com" + e["data"]["permalink"] + ".json"
        j=requests.get(url, headers=HEADERS).json()
        print ("len(j)=%d" % len(j), end=" | ")
        print ("[0] data children -> len()=%d" % len(j[0]["data"]["children"]) , end=" | ")
        print ("[1] data children -> len()=%d" % len(j[1]["data"]["children"]) , end=" | ")
        print (url)


def comments():
    print ("\nComments:\n")
    understandCommentsFormat("https://www.reddit.com/r/history/comments/b9zbs7/-/ek9aajm/.json") # comment missing
    understandCommentsFormat("https://www.reddit.com/r/history/comments/b9zbs7/-/ek99tfe/.json") # comment not missing
    understandCommentsFormat("https://www.reddit.com/comments/ba2a3a/-/ek8kc0n/.json") # has no selftext!
    # understandCommentsFormat("https://www.reddit.com/comments/b9zbs7/-/ek89uuj/.json") # incl selftext
    checkManyComments()

def understandThreadOpFormat(url):
    print (url)
    r=requests.get(url, headers=HEADERS)
    j=r.json()
    print ("type(j)=%s | len(j)=%d" % (type(j).__name__,len(j)))
    print ("keys()", j.keys())
    print ("kind:", j["kind"])
    print ("data, keys:", j["data"].keys())
    children = j["data"]["children"]
    print ("data, children --> len():", len(children))
    print ("data, children, [0], keys():", children[0].keys())
    print ("data, children, [0], kind:", children[0]["kind"])
    print ("data, children, [0], data, keys(): ", sorted(children[0]["data"].keys()))
    data = children[0]["data"]
    print ("name: %s" % (data["name"]))
    print ("author: %s" % (data["author"]))
    print ("title: %s" % (data["title"]))
    print ("selftext: %s" % (data["selftext"]))


def checkManyThreadOps(jsonfile="comments.json"):
    with open(jsonfile) as f:
        j = json.load(f)

    doneAlready=[]
    for e in j:
        url = "https://www.reddit.com/by_id/%s/.json" % e["data"]["link_id"]
        if url in doneAlready:
            continue
        doneAlready.append(url)
        j=requests.get(url, headers=HEADERS).json()
        print ("len(j)=%d" % len(j), end=" | ")
        children = j["data"]["children"]
        print ("data, children --> len()=%d" % len(children), end=" | ")
        print (url)


def threadops():
    print ("\nThread OPs:\n")
    # understandCommentsFormat("https://www.reddit.com/by_id/t3_b9zbs7/.json")
    understandThreadOpFormat("https://www.reddit.com/by_id/t3_b9zbs7/.json")
    print ()
    checkManyThreadOps()

def main():
    comments()
    print ("\n" + 50*"#" +"\n")
    threadops()

if __name__ == '__main__':
    main()
