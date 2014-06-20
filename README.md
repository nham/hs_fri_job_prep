# phbo
A program for managing phonebooks.

## Setup
Before running the database must be initialized by calling init_db(). Perhaps the best way is to use the Python REPL:

    Python 3.4.1 (default, May 19 2014, 17:23:49) 
    [GCC 4.9.0 20140507 (prerelease)] on linux
    Type "help", "copyright", "credits" or "license" for more information.

    >>> import phbo
    >>> phbo.init_db()


## Default phonebook
A default phonebook can be configured by adding the following to phbo.cfg:

    [DEFAULT]
    default_phonebook = phonebook_name_here
