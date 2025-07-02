Kitten.rs, my first deployed website, was a naive attempt to create a public webserver without 
relying on random dependancies. I had intended for the site to only require the absolutely 
necessary dependencies as a measure to keep compile-time low and partially because I was 
curious how hard it would be to create a web-server. I only needed something simple to park 
on my domain until I figured out what I actually wanted there.

# A Lesson on 'Temporary' Becoming Permanent

The web-server - henceforth referred to as just "Kitten" - was intended for two things: 
exist without maintenance, and provide a place for me to publish my ramblings. It 
succeeded at those two things but only ever those two things, projects couldn't actually 
be shown off since every page was constructed with markdown and I couldn't just slide the 
required tags in there easilly. All of this was fine since I had was "going to make it better 
later", of course "later" never actually came and simple features like distributing 
image files had to be cut down as to not create any vulnerabilities. This project alone has 
taught me to never settle for temporary when permanent is available purely because of the 
headaches I received from trying to add Discord verification files, and each viewing of 
the code started to feel more and more like some esoteric torture method. 

# Shortcomings 

## Routing

Routing was a 182 line match statement.

My choice for routing was a complete mistake, every route was completely manual, and 
adding a new directory would require a full re-write of the routing just to allow files 
to be spread accross different folders. Images had their mime-type set in a match statement 
that only covered: png, jpg, and webp, anything else just got given some random default type.
I'm fairly certain I still haven't fixed everything to this day, I'm pretty sure delivering 
fonts to the user is just completely broken and I never thought to go and fix it because the 
routing code deals mental damage over time.

## Project Showcase

This 

## Too Static
