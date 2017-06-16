# Morrissey Chat Room

## What is this?

There's a fairly new idea called Helix that's making waves in the Rust and Ruby communities.
This is mainly because the extension allows for C-like extension behavior by leveraging Rust
to make Ruby / Rails applications faster.

This is useful for a couple of reasons, but I wanted to check out how everything actually works.
As a mutation of their original tutorial on the extension, I decided to see what it would be like
to create a "chat bot" of sorts that had the personality of Morrissey (of the Smiths).

## What Features Are In It?

Right now, just a simple form that allows you to interface with the chat bot. The page reloads
every time you make a query, so extending this to become a client-side chat application would be
pretty neat.

## Dependency Information

* Rails 5.1
* Rust Nightly

## Deploying This

If you're rolling with Heroku, this process is fairly straightforward:

1. `heroku create`
2. `heroku buildpacks:add https://github.com/hone/heroku-buildpack-rust`
3. `heroku buildpacks:add heroku/ruby`
4. Push to your heroku origin - `git push heroku [branch]`
