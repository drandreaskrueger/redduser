
example with whole trail of replies:
```
curl --header "User-agent: redduser" https://www.reddit.com/r/politics/comments/bdwg0g/-/el29x7x/.json > test.json
cat test.json | jq

cat test.json | jq '.[0].kind'
cat test.json | jq '.[0].data | keys'
cat test.json | jq '.[0].data.children[0].data | keys '
cat test.json | jq '.[0].data.children[0].data.name'
cat test.json | jq '.[0].data.children[0].data.title'
cat test.json | jq '.[0].data.children[0].data.selftext'

cat test.json | jq '.[1].data | keys'
cat test.json | jq '.[1].data.children | length'

cat test.json | jq '.[1].data.children[0].data | keys'
cat test.json | jq '.[1].data.children[0].data.name'
cat test.json | jq '.[1].data.children[0].data.body'

cat test.json | jq '.[1].data.children[0].data.replies.data.children | length'
cat test.json | jq '.[1].data.children[0].data.replies.data.children[0].data | keys'
cat test.json | jq '.[1].data.children[0].data.replies.data.children[0].data.body'

cat test.json | jq '.[1].data.children[0].data.replies.data.children[0].data.replies.data.children | length'
cat test.json | jq '.[1].data.children[0].data.replies.data.children[0].data.replies.data.children[0].data | keys'
cat test.json | jq '.[1].data.children[0].data.replies.data.children[0].data.replies.data.children[0].data.body'
cat test.json | jq '.[1].data.children[0].data.replies.data.children[0].data.replies.data.children[0].data.name'
```

the last one, "t1_el2dl6f", turned into a URL and download:
```
curl --header "User-agent: redduser" https://www.reddit.com/r/politics/comments/bdwg0g/-/el2dl6f/.json > test.json
cat test.json | jq '.[1].data.children[0].data'
cat test.json | jq '.[1].data.children[0].data.name'
cat test.json | jq '.[1].data.children[0].data.replies'
```

comment with OP that has a title AND a selftext:
```
curl --header "User-agent: redduser" https://www.reddit.com/comments/b4znbi/-/ejg8fwk/.json | jq '.[0].data.children[0].data | .title + "##  ##" + .selftext'
```
so looks like the OP is always fully included in the comment.
