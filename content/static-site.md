## making your own static site with rust

I love static site generators! 

some of my friends might remember that i used to use [quartz](https://quartz.jzhao.xyz/) a lot, and it is a really good static site generator! its really pretty on default and easy to configure. if anyones getting into making their own site and looking for a generator i would really reccommend it!

But i guess one day i decided to work on making my own, since quartz has some of it's own quirks (i guess) that wouldn't allow me to configure things exactly the way i wanted.

Unfortuantely i dont know much web devlopment tools/frameworks. I mean i guess i can vibe code something up but i wanted to work on something that i can call my own, Not like im against using AI agents for coding projects but i wanted this personal website to feel like me (also why these blogs are not very formal and just thought dumps). 

So everythings raw html and css!

For the actual website generating part, i used Rust. Rust is a cool language and all but i struggle using it and i would love to get better at writing rust code. I used the [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark) crate for the markdown parsing. 

In my final version (that is *very* slowly being worked on) of this static site generator, there are features such as table of contents/light dark mode/etc that would hopefully bring a better user experience. A tool that i have been using to work on this is [serde](https://github.com/serde-rs/serde) crate, which is a *framework for serializing and deserializing Rust data structures*, for quick JSON conversion, to save data from your code (i use it to keep track of the current files and other misc objects).    

until i learn more javascript lol