package ds

// 效率不高的字典树
type Trie struct {
	arr  [][26]int
	cnt  []int //非0表示有多少个下标i
	size int
}

func MakeTrie() Trie {
	return Trie{arr: make([][26]int, 1), cnt: make([]int, 1), size: 1}
}

func (trie *Trie) Insert(word string) {
	cur := 0
	for i := range word {
		if trie.arr[cur][word[i]-'a'] == 0 {
			trie.arr[cur][word[i]-'a'] = trie.size
			trie.arr = append(trie.arr, [26]int{})
			trie.cnt = append(trie.cnt, 0)
			trie.size++
		}
		cur = trie.arr[cur][word[i]-'a']
	}
	trie.cnt[cur]++
}

func (trie *Trie) Search(word string) bool {
	cur := 0
	for i := range word {
		if trie.arr[cur][word[i]-'a'] == 0 {
			return false
		}
		cur = trie.arr[cur][word[i]-'a']
	}
	return trie.cnt[cur] > 0
}

func (trie *Trie) StartsWith(prefix string) bool {
	cur := 0
	for i := range prefix {
		if trie.arr[cur][prefix[i]-'a'] == 0 {
			return false
		}
		cur = trie.arr[cur][prefix[i]-'a']
	}
	return true
}
