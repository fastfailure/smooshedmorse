# Smooshed Morse

[Smooshed Morse] exercise implemented while learning Rust

[Smooshed Morse]: https://www.reddit.com/r/dailyprogrammer/comments/cmd1hb/20190805_challenge_380_easy_smooshed_morse_code_1/

Usage:

        smooshedmorse encode <English word>
        sdecodemooshedmorse decode <Smooshedmorse word>
        smooshedmorse [extra1|extra2|extra3|extra4]
        smooshedmorse permutations [<smooshedmorse alphabet permutation>]

`permutations` command implement [smooshedmorse challenge 2](https://www.reddit.com/r/dailyprogrammer/comments/cn6gz5/20190807_challenge_380_intermediate_smooshed/)

If no alphabet permutation is given a random one is used.

 Examples:

        smooshedmorse encode Horse
        smooshedmorse decode ....---.-.....
        smooshedmorse permutations .--...-.-.-.....-.--........----.-.-..---.---.--.--.-.-....-..-...-.---..--.----..
