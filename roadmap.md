# Mist Roadmap

## 0.1.0
- [X] implement http protocol
    - [X] header
    - [X] request
    - [X] response
    - [X] status code
    - [X] uri (url)
    - [X] method
- [ ] parse http protocol
    - [X] header
    - [X] request
    - [ ] response
    - [X] status code
    - [X] uri (url)
    - [X] method

## Features
- [ ] parse http protocol
    - [X] url parsing (via squid)
- [ ] construct requests
    - string-based
    - code-based

## API
- [X] Request struct
- [X] Response struct
- [X] Request parser
- [ ] Response parser
- [ ] docs for all standard headers and methods

## Long-term goals
- [X] no alloc crate
- [ ] feature parity with http crate
- [ ] ability to fully construct headers with code
    - build up status line + headers (w/ values)
    - collect into request/response struct w/ body
    - see kawa crate
