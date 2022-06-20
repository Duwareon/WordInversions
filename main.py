#!/usr/bin/env Python3
def treat_list(text, k):
     
    # create the empty list
    words = []
     
    for x in text:
        # if length of current sub string
        # is greater than k then
        if len(x) > k:
             
            # append this sub string in
            # string list
            words.append(x)
             
     # return string list
    print(len(words))
    return words

def load_words():
    with open('words_alpha.txt') as word_file:
        words = word_file.read().split()

    return treat_list(words, 4)

def invert(w):
    return w[1:]+w[0]


def compare_words(w1, w2):
    if w1 == w2:
        return (False, 0)
    if len(w1) != len(w2):
        return (False, 1)

    for i in range(0, len(w2)):
        if w1 == w2:
            return (True, i)
        w2 = invert(w2)
    return (False, 2)

def main():
    words = load_words()
    for w1 in words:
        for w2 in words:
            comp = compare_words(w1, w2)
            if comp[0]:
                print(w1, w2, comp[1], sep=" ")

main()