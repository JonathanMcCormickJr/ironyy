# Ironyy

A CLI project management tool. 

Written by Jonathan McCormick Jr. and LGR as part of Jonathan's studies with the LGR Rust Developer Bootcamp.

## TODO
- Complete the base requirements for the project management tool, including all lingering TODOs not in this list.
- Write comprehensive tests for all modules and functionalities.
- Test for unexpected inputs and edge cases, including user options which may apply in one page, but not another.
- Implement a client-server architecture with a REST API.
- Implement user accounts, login/logout, and data ownership, including passwords and TOTP.
- Implement password-backed, PQ, E2E encryption.


## Deviances

### Database Handling
While the project originally contained several layers of abstraction for database handling, I have decided to simplify the architecture to have a `.json` file as the database, with a single `DBState` struct to manage interactions with it. This was done to reduce complexity and make the code more maintainable.

```text
Initial read of JSON file
----------    -----------    --------------
|  App   | <- | DBState | <- | JSON file  |
----------    -----------    --------------

Subsequent reads do not re-read the JSON file
----------    -----------    --------------
|  App   | <- | DBState |    | JSON file  |
----------    -----------    --------------

All writes pass through DBState to JSON file
----------    -----------    --------------
|  App   | -> | DBState | -> | JSON file  |
----------    -----------    --------------

```

### UI Prompts / Actions
The original design included a `Prompts` struct with dynamic dispatch for various user prompts. However, this added unnecessary complexity. I have simplified the design to use a more straightforward approach without dynamic dispatch, as the set of functions is fixed and does not require such flexibility.

```text

 ####################           ####################
 #  User input      # --------> # Match input to   #
 #                  #           # function calls   #
 ####################           ####################
                                         /|         
   Functions change state of Nav and DB / |        
           /---------------------------/  |         
          |                               |         
          V                               V         
 ####################           ####################
 #    Navigator     #           #    DBState       #
 #                  #           #                  #
 ####################           ####################
          |                              /|          
          | Both Nav and DB change UI   / | DBState automatically
          |/---------------------------/  | writes to JSON file       
          |                               | to ensure persistence
          V                               V         
 ####################           ####################
 #       UI         #           #   JSON file      #
 #                  #           #                  #
 ####################           ####################
                                                    
```
