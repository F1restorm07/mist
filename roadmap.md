# Mist Roadmap

## 0.1.0
- [ ] implement http protocol
    - [X] header
    - [X] request
    - [X] response
    - [X] status code
    - [X] uri (url)
    - [X] method
- [ ] parse http protocol
    - [ ] header
    - [ ] request
    - [ ] response
    - [ ] status code
    - [X] uri (url)
    - [ ] method

## Features
- [ ] parse http protocol
    - [ ] url parsing (via squid)
- [ ] construct requests
    - string-based
    - code-based

## API
- [X] Request struct
- [X] Response struct
- [ ] Request parser
- [ ] Response parser
- [ ] Header parser
- [ ] docs for all standard headers and methods

## Long-term goals
- [ ] no alloc crate
- [ ] feature parity with http crate
- [ ] ability to fully construct headers with code
    - build up status line + headers (w/ values)
    - collect into request/response struct w/ body
    - see kawa crate
