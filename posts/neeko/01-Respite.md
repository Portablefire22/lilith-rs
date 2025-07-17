Let's set the scene for this project. It's the 17th of January 2025 and YouTube started to recommend various 
videos about the now defunct "ChronoShift" project - videos of which can be found 
[here](https://www.youtube.com/playlist?list=PLfVEn_PNuKhDQjnsdOVGVByutEilSkzLK). I had always been acutely aware of the 
ChronoShift project but never thought much of it until I remembered a 
[Reddit post about it being shutdown](https://www.reddit.com/r/leagueoflegends/comments/u7u7hv/chronoshift_an_emulation_of_2011_league_of/),
it saddened me quite a bit since I've always been interested in playing old versions of League of Legends. Hell, it 
got to the point that it ***started appearing in my damn dreams***, and so I took it upon myself to get a part of 
old League of Legends functional again.

# Getting the Game Files 

Riot, like many other companies, apparently don't like it when you try to re-create their game, and so they don't just 
provide these files publicly anymore. Apparently they used to provide these files through an old update service that 
they just never deactivated, allowing anyone to download all game files from release to patch v9.13, until they decided 
to disable the servers after sending the legal threats that shut down Chronoshift.

Thankfully, these game files are still accessible through the 
[Riot Archive Project](https://www.reddit.com/r/leagueoflegends/comments/yqnqzq/the_riot_archive_project_making_10_years_of/) - 
an unofficial archive of all Riot Games content - and all I needed to do was wait for a 241GB archive to download.

![241 Gigabyte archive downloading at 688 Kilobytes per second](/public/blogs/neeko/respite/archive_download.png)

Five days after clicking download and I could finally open the files. I spent the first couple of hours just searching 
through various directories in hopes of finding some weird assets, those hopes were fulfilled when I found a very early 
asset for the taunt crowd control. 

![poorly drawn fat guy pointing down with the text "taunt" written next to him](/public/blogs/neeko/respite/taunt.png)

Eventually I got bored with looking through the game files and decided to run the actual launcher for the game. A few 
misfires from trying to launch "LolLauncher.exe" instead of "LolClient.exe", plus some additional tinkering with 
"Adobe Air.dll" later and I finally got the Adobe AIR client opened.

![League of Legends Adobe Air client opened to a Maestro connection error](/public/blogs/neeko/respite/maestro_error.png)

Unfortunately the client doesn't function without something called "Maestro" and I'm sure it's missing plenty of other 
connection, so I guess it's time to actually start figuring out how everything works. From this point onwards, everything 
will be separated into their own posts as a way to try to avoid confusion. This method was mostly chosen since development 
was performed in such a manner that writing the chronological journey would result in a giant mess, I consistently 
hopped between different parts of the project and routinely forgot to document what I did :)

# Next Steps 

Okay so we actually need to create things for this client to work, the first step was to create this "Maestro" the client 
kept complaining about, but I thought that some preliminary information was probably best included first. If you want 
chronological order then read maestro before anything else, otherwise you can read the next posts in the following order:

- [RTMP](/blog/RTMP)
- [AMF](/blog/Action%20Message%20Format)
- [Maestro](/blog/Khada)
- [Nexus](/blog/Nexus)
- [Neeko](/blog/Neeko%20Server)