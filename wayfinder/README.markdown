wayfinder
=========

a little http router generator

- what is it?
- getting started
- language reference
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

```
use uuid::Uuid;

/
  [lang: String]

  users
    GET -> People::Index

  people
    GET People::Index
    POST People::Create

    new
      GET People::New

    {id: Uuid}
      GET People::Show
      PUT People::Update
        [name: String]
      DELETE People::Destroy

      edit
        GET People::Edit
```

getting started
---------------

Look at the basic example configurations in `examples.routes` and
`common.routes`.

Check out the example application in `examples/cli/`, which
runs the routing algorithm as a CLI.  Try out:

```
> cargo run /books/
```

language reference
------------------

A route configuration file is composed of two parts: an optional
header and the main hierarchical route configuration section.  The
header can be any Rust code to be passed through to the generated
module, mainly to `use` any types referred to below.

The route configuration starts with a line containing the single
character '/'.  The routes form a hierarchy, and the structure is
recursively defined.  It is whitespace-sensitive, with indentation
level corresponding to nesting level.  Blank lines can be added
anywhere.

Each route segment can have three types of children: query parameters,
resources, and nested routes.  They must be specified in that order.

Query parameters are a name-type pair written inside square brackets,
like so: `[lang: String]`.  They apply to every resource on that
route and every nested route.

Resources are particular HTTP verbs that your application will
respond to.  They consist of two required parts and an optional one.
The verb itself is listed first, followed by the name of the resource.
If the route should redirect to that resource rather than directly
serving it, an arrow can be written between the parts.  So a simple
resource might look like `GET people`, and a redirect `GET -> people`.
Resources can also have query parameters, they are written in a block
nested under the resource.

Nested routes come last.  The consist of a path segment followed by
a nested block of query parameters, resources, and routes.  The
path segment can be either a static string (e.g. `people`) or a
path parameter written between curly braces, like `{id: Uuid}`.

more information
----------------

See the generated documentation for usage help, both for this
module as well as for the generated routes.
