# Example outputs
### Example when no parameters are given:
```
python3 redduser.py
Arguments given: (days=2, file='comments.json', user='andreaskrueger')
Downloading until I see a post which was before '20190324-201225'

https://www.reddit.com/user/andreaskrueger/comments/.json?limit=4&after=None <Response [200]>
20190326-125545  +3 r/ABoringDys reddit.com/comments/b5ibwu/-/ejew7v4  &gt; you have to raise $1,000  He couldn
20190325-230111  +1 r/cscareerqu reddit.com/comments/b59yw6/-/ejdjs57  Cannot imagine not having learned it.
20190325-221822  +4 r/collapse   reddit.com/comments/b5bjym/-/ejdg3zl  20-30 years too late. That's why "democr
20190325-060231  +1 r/entertainm reddit.com/comments/b4njal/-/ejblaiu  The **500x is pure propaganda**. It's in

https://www.reddit.com/user/andreaskrueger/comments/.json?limit=4&after=t1_ejblaiu <Response [200]>
20190325-051950  +1 r/entertainm reddit.com/comments/b4njal/-/ejbjatf  Agreed, but **the 500x is pure propagand
20190325-040452  +1 r/collapse   reddit.com/comments/b4yb7e/-/ejbf79i  Hierarchy.
20190325-040014  +2 r/collapse   reddit.com/comments/b4yb7e/-/ejbex6q  &gt; locked collars on their security te
20190325-035208  +1 r/WhitePeopl reddit.com/comments/b50mmx/-/ejbef2g  And now [they build doomsday bunkers for

https://www.reddit.com/user/andreaskrueger/comments/.json?limit=4&after=t1_ejbef2g <Response [200]>
20190325-034955  +6 r/collapse   reddit.com/comments/b4yb7e/-/ejbea7t  &gt; intervene  Oh, intervened they have
20190325-033504  +1 r/WhitePeopl reddit.com/comments/b50mmx/-/ejbdc00  Plus:  - a disconnected caste which live
20190324-181714  +1 r/DataHoarde reddit.com/comments/b35zfh/-/eja52s4  Thanks for the docs hint. I have found t
20190324-045930  +1 r/Catastroph reddit.com/comments/b4r207/-/ej8x9lz  Buying cheap stuff on eBay and amazon.

Oldest seen post was at '20190324-045930' which is more than 2 days ago. Breaking from the loop.

Done. Written 12 comments to file 'comments.json' (32937 bytes).
```

### Help for parameters:
```
python3 redduser.py -h

usage: redduser.py [-h] [--user USER] [--days DAYS] [--file FILE]

Download recent reddit-comments of a user. Version v0.1.

optional arguments:
  -h, --help   show this help message and exit
  --user USER  reddit username (default: andreaskrueger)
  --days DAYS  go back how many days? (default: 2)
  --file FILE  output file (default: comments.json)
```

### Example for a user with very few (2) total posts:
```
python3 redduser.py --user Stegoserious --days 1000 --file Stegoserious.json

Arguments given: (days=1000, file='Stegoserious.json', user='Stegoserious')
Downloading until I see a post which was before '20160629-202700' UTC.

https://www.reddit.com/user/Stegoserious/comments/.json?limit=4&after=None <Response [200]>
20190326-145851    +28 r/bestof     reddit.com/comments/b5k9jr/-/ejf5zlq  Yes, this is my actual first post.  No, 
20190325-223716 +10843 r/dataisbeau reddit.com/comments/b5f9wi/-/ejdhqf8  I'm probably in the top 5% of longest lu
Returned pagination parameter 'after' was 'None' = No more posts! Breaking from the loop.

Done. Written 2 comments to file 'Stegoserious.json' (6176 bytes).
```


# JSON processing

all comments end up in one file which can then be post-processed, e.g. with jq:

show all dates:

    cat comments.json | jq ".[] | .data.created_utc" | xargs -i date -d "@{}" "+%F" | sort | uniq

all links:

    cat  comments.json | jq '.[] | .data.permalink'

subreddit names + post titles

    cat comments.json | jq '.[] | .data.subreddit +" | " + .data.link_title '

etc. Please add more, thanks.