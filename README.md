# RandomSmallRustProjects
Random Rust Projects that don't need their own repo

## tracing_stdout_stderr
This is a project that was experimenting with tracing lauers.
With two layers attached to the same registery, you can log to both stdout and stderr.
This was not explained terribly well in the docs, but I did eventually fumble my way through it.

* to run
`cargo run --bin tracing_stdout_stderr`

## warp_ip_response
This is a small webserver I created to experiment with warp's filters, and to better understand how they work
It returns the ip address when called with a `key=secret` param with no query path
If called with `/get/headers?key=secret` it will return all of the headers.

* to run on localhost
LOCAL=1 cargo run --bin warp_ip_response

* env variables:
    * SECRET=SECRET
    * PORT=8000
    * LOCAL=
