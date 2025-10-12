# Ironyy

A CLI project management tool. 

Written by Jonathan McCormick Jr. and LGR as part of Jonathan's studies with the LGR Rust Developer Bootcamp.

## TODO
- Make user input case-insensitive.
- Implement a client-server architecture with a REST API.
- Implement user accounts, login/logout, and data ownership, including passwords and TOTP.
- Implement password-backed, PQ, E2E encryption.


## Deviances

### Database Handling

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
