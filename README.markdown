wayfinder
=========

a little http router generator

- what is it?
- getting started
- more information

what is it?
-----------

Routing configuration is almost universally done dynamically during
application initialization, even in systems that tend towards a
static style.  But the lines of change of routing are much slower
than runtime, and we can make things much easier on ourselves by
accepting that -- we get static typing of our routes *for free*!

The philosophy of `wayfinder` is to compile routes at build time,
allowing a server to route with code that performs as well as a
hand-written router.  Isn't that what we all want?

We also declare the route parameter types ahead of time, so the
code to parse them is produced automatically, and our application
code can focus on the domain level.

getting started
---------------

Look at the basic example configurations in `examples.routes` and
`common.routes`.

Check out the example application in `examples/cli/`, which
runs the routing algorithm as a CLI.  Try out:

```
> cargo run /books/
```

more information
----------------

See the generated documentation for usage help, both for this
module as well as for the generated routes.
