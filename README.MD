# A toy LISP interpreter

This is a work in progress.

Currently it can run a GCD algorithm:
```lisp
(global gcd 
    (fn (a b) 
        (with (_a b _b (mod a b)) 
            (if (== _b 0) 
                _a 
                (gcd _a _b)))))
```

TODO:
- [ ] Implement actual error handling
- [ ] Plug in a console crate
- [ ] Implement proper logging
- [ ] Store things like spans in AST for better error messages?
- [ ] Add more functions to native stdlib
- [ ] Add the ability for user-implemented macros
- [ ] Add the non-interactive mode for running files