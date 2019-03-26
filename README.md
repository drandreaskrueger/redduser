# redduser v0.1
download all my comments from reddit

## run 

    python3 redduser.py

and get help with

    python3 redduser.py -h

see [examples.md](examples.md) for example outputs.

## discussion

This little tool was inspired ... by a lack of something alike. See:

* ["download my stuff /u/username - where is this reddit functionality?"](https://www.reddit.com/r/DataHoarder/comments/b35zfh/download_my_stuff_uusername_where_is_this_reddit/)
* ["rust libraries for reddit API ?"](https://www.reddit.com/r/rust/comments/b4znbi/rust_libraries_for_reddit_api/)

All still done in Python though, much faster to develop; for me at least.

## related 

* tools
  * [aliparlakci/bulk-downloader-for-reddit](https://github.com/aliparlakci/bulk-downloader-for-reddit/issues/59)
  * [MalloyDelacroix/DownloaderForReddit](https://github.com/MalloyDelacroix/DownloaderForReddit/issues/59)
  * [DrPugsley/Reddit-Archive-Host](https://github.com/DrPugsley/Reddit-Archive-Host/issues/2)
* libraries
  * [zpallin/rust_reddit](https://github.com/zpallin/rust_reddit)
  * [IntrepidPig/orca](https://github.com/IntrepidPig/orca)

## next

Download the parent comment to each of mine. Multi-threaded. Perhaps try that in rust then?

Pretty printing, formatting as ... webpage perhaps? What would be the best language for that?

All threads started by me are missing. Add them in; but the json structure might be very different then. 