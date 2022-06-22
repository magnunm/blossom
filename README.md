# blossom - A Bloom filter

A Bloom filter is a probabilistic data structure that can be used to represent
a set. When testing for membership the Bloom filter will either tell you that
"the element is definitely not in the set" or "the element is maybe in the
set". The advantage of the Bloom filter over set representations with a
deterministic membership test it that it can often be much smaller in size. The
size of the bloom filter is for example independent of both the size of the
inserted elements and the number of elements inserted. At the same time both
insertions and membership tests have constant time complexity. Elements can not
be removed from a Bloom filter.

The probability of false positives can be reduced, at the cost of increasing
the size of the Bloom filter and slowing down insertions and membership tests.
A convenience method is provided to construct a Bloom filter with an upper
bound on the false positive probability.
