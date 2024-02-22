package gosol

import "container/heap"

const (
	feedLength = 10
)

type Twitter struct {
	userTweets  map[int][]Tweet
	follows     map[int]map[int]struct{} // key follows values
	globCounter int
}

type Tweet struct {
	id        int
	globCount int
}

func Constructor() Twitter {
	return Twitter{
		userTweets:  make(map[int][]Tweet),
		follows:     make(map[int]map[int]struct{}),
		globCounter: 0,
	}
}

func (this *Twitter) PostTweet(userId int, tweetId int) {
	this.userTweets[userId] = append(this.userTweets[userId], Tweet{
		id:        tweetId,
		globCount: this.globCounter,
	})
	this.globCounter += 1
}

func (this *Twitter) GetNewsFeed(userId int) []int {
	newFeed := new(NewFeed)
	heap.Init(newFeed)

	// add user posts
	for _, post := range this.userTweets[userId] {
		heap.Push(newFeed, post)
	}

	// add follow posts
	for followee := range this.follows[userId] {
		for _, post := range this.userTweets[followee] {
			heap.Push(newFeed, post)
		}
	}

	var feedIds = make([]int, 0, feedLength)
	for i := 0; i < feedLength && newFeed.Len() > 0; i++ {
		tweet := heap.Pop(newFeed).(Tweet)
		feedIds = append(feedIds, tweet.id)
	}

	return nil
}

func (this *Twitter) Follow(followerId int, followeeId int) {
	if _, ok := this.follows[followerId]; !ok {
		this.follows[followerId] = make(map[int]struct{})
	}
	this.follows[followerId][followeeId] = struct{}{}
}

func (this *Twitter) Unfollow(followerId int, followeeId int) {
	if _, ok := this.follows[followerId]; !ok {
		this.follows[followerId] = make(map[int]struct{})
	}
	delete(this.follows[followerId], followeeId)
}

type NewFeed []Tweet

func (n NewFeed) Len() int {
	return len(n)
}

func (n NewFeed) Less(i, j int) bool {
	return n[i].globCount > n[j].globCount // MaxHeap
}

func (n *NewFeed) Swap(i, j int) {
	(*n)[i], (*n)[j] = (*n)[j], (*n)[i]
}

func (n *NewFeed) Push(x any) {
	(*n) = append((*n), x.(Tweet))
}
func (n *NewFeed) Pop() any {
	l := n.Len()

	v := (*n)[l-1]
	(*n) = (*n)[:l-1]

	return v
}

/**
 * Your Twitter object will be instantiated and called as such:
 * obj := Constructor();
 * obj.PostTweet(userId,tweetId);
 * param_2 := obj.GetNewsFeed(userId);
 * obj.Follow(followerId,followeeId);
 * obj.Unfollow(followerId,followeeId);
 */
