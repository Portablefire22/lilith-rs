Discord, like most companies these days, have released an application to the public that uses the 
Electron framework to ship a website as a native application. Fair enough to be honest, I've done 
GUI projects before and the only one that didn't make want to kill myself was working with Electron 
to create a simple music player; it's just so strange that they decide to completely restrict the 
user in what they can do. They've completely disabled the developer tools in the standard client 
and explicitly forbid any modification to their client in the TOS - they act like third-party 
modifications could do anything that they don't allow when the data can only be accessed through 
their own API? Then they hide behind their TOS forbidding third-party clients whilst providing 
the documentation for how to access the same API that the first-party client uses all because 
their bot API is the exact same as the user API; at this point they're just asking for people to 
try and make a third-party client. So here we are, let's try and make a third-party client.

# Tech 

I have a semi-active Golang project going on right now and it has occupied the last 131 hours 
(as of writing this document) that I have dedicated to programming. I've come to enjoy the 
language and will probably use it whenever the thought of Python would usually cross my mind -
it has type-safety, error checking, and it just works but it's not Rust. Something about Rust 
just scratches an itch inside of me, I will abandon C++ projects just to go write something in 
Rust because I haven't had a hit of Rust in a while, it was the language that re-ignited my 
passion for programming and it has just been pulling me back in everytime I try something new. 
So I loaded up ["Are We Gui Yet"](https://areweguiyet.com) and narrowed down all of the options 
to: GTK, Tauri, or "just" writing a "small" Vulkan application that displays a GUI - with the 
reasoning being that I needed something that worked cross-platform, since many people aren't 
developing Linux only GUI libraries. I'm not a designer - nor do I wish to sit down and learn 
UI design - so I'd prefer if the GUI library also had some design library available to me, 
leaving GTK and Tauri. GTK kicked my arse on a different project so I chose 
[Tauri](https://tauri.app/) :) 

## Tauri 

If you don't know what Tauri is then just think of your average Electron application but without 
an instance of Google Chrome bundled within. For our purposes it's just Electron but with 
Rust instead of [Java/Type]Script as the backend language. Front-end is entirely my choice and 
since I'm not a masochist, I opted for React + Material UI with Typescript as the language of 
choice. I'd prefer if it was all Rust but my experience with Rust in GUI scenarios isn't the 
greatest, as the All-Seeing Borrowchecker loves to just eviscerate me over the slightest fuck-up, 
and I'm not away of any Rust web UI libraries that would fix my "I'm not a designer" problem.

# The API Library

I ***really*** shouldn't have done this.

