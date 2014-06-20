#! /usr/bin/python

import sys, sqlite3, configparser

db_name = "pbdb.sqlite"

class phonebookException(Exception):
    def __init__(self, value):
        self.value = value


class argParseException(phonebookException):
    pass

class commandException(phonebookException):
    pass


def init_db():
    conn = sqlite3.connect(db_name)

    init_sql = """
    CREATE TABLE phonebooks (
      id integer primary key autoincrement,
      name text UNIQUE);

    CREATE TABLE pb_entries (
      id integer primary key autoincrement,
      name text,
      number text,
      pbid int);
      """

    conn.executescript(init_sql);
    conn.commit()


class PhonebookManager:
    def __init__(self, conn):
        self.conn = conn

    # if the phonebook exists, return its id (from the table)
    # else return False
    def pb_exists(self, pb_name):
        c = self.conn.cursor()
        sql = "SELECT id FROM phonebooks WHERE name = ?"
        c.execute(sql, (pb_name,))
        result = c.fetchone()

        if result == None:
            return False
        else:
            return result[0]

    # checks if <name> is already in the phonebook associated with <pbid>
    def name_in_list(self, name, pbid):
        c = self.conn.cursor()
        sql = "SELECT id FROM pb_entries WHERE name = ? and pbid = ?"
        c.execute(sql, (name, pbid))
        result = c.fetchone()
        return result is not None


    def create_phonebook(self, pb_name):
        if self.pb_exists(pb_name):
            raise commandException("Cannot create phonebook '{}' because it already exists".format(pb_name))

        c = self.conn.cursor()
        sql = "INSERT INTO phonebooks (name) VALUES (?)"
        c.execute(sql, (pb_name,))
        self.conn.commit()


    def lookup_name(self, name, pb_name):
        pbid = self.pb_exists(pb_name)
        if not pbid:
            raise commandException("Phonebook '{}' does not exist".format(pb_name))

        c = self.conn.cursor()
        sql = "SELECT name, number FROM pb_entries WHERE pbid = ? and name LIKE ?"
        c.execute(sql, (pbid, '%'+name+'%'))

        result = c.fetchone()
        while result != None:
            self.display_entry(result[0], result[1])
            result = c.fetchone()


    def add_entry(self, name, number, pb_name):
        pbid = self.pb_exists(pb_name)
        if not pbid:
            raise commandException("Phonebook '{}' does not exist".format(pb_name))

        if self.name_in_list(name, pbid):
            raise commandException("'{}' is already added to phonebook '{}'".format(name, pb_name))


        c = self.conn.cursor()
        sql = "INSERT INTO pb_entries (name, number, pbid) VALUES (?, ?, ?)"
        c.execute(sql, (name, number, pbid))
        self.conn.commit()


    def change_entry(self, name, number, pb_name):
        pbid = self.pb_exists(pb_name)
        if not pbid:
            raise commandException("Phonebook '{}' does not exist".format(pb_name))

        if not self.name_in_list(name, pbid):
            raise commandException("'{}' isn't in phonebook '{}', so it cannot be changed.".format(name, pb_name))

        c = self.conn.cursor()
        sql = "UPDATE pb_entries SET number = ? WHERE pbid=? and name=?"
        c.execute(sql, (number, pbid, name))
        self.conn.commit()



    def remove_entry(self, name, pb_name):
        pbid = self.pb_exists(pb_name)
        if not pbid:
            raise commandException("Phonebook '{}' does not exist".format(pb_name))

        if not self.name_in_list(name, pbid):
            raise commandException("'{}' isn't in phonebook '{}', so it cannot be removed.".format(name, pb_name))

        c = self.conn.cursor()
        sql = "DELETE FROM pb_entries WHERE name = ? and pbid = ?"
        c.execute(sql, (name, pbid))
        self.conn.commit()
        


    def reverse_lookup(self, number, pb_name):
        pbid = self.pb_exists(pb_name)
        if not pbid:
            raise commandException("Phonebook '{}' does not exist".format(pb_name))

        c = self.conn.cursor()
        sql = "SELECT name, number FROM pb_entries WHERE pbid = ? and number = ?"
        c.execute(sql, (pbid, number))

        result = c.fetchone()
        while result != None:
            self.display_entry(result[0], result[1])
            result = c.fetchone()


    def display_entry(self, name, number):
        print("{}: {}".format(name, number))



def exec_command(args, default_phonebook=None):
    if len(args) == 0:
        raise argParseException("No command specified.")

    # create <phonebook>
    # lookup <name> <phonebook>
    # add <name> <number> <phonebook>
    # change <name> <number> <phonebook>
    # remove <name> <phonebook>
    # reverse-lookup <number> <phonebook>

    def check_num_params(cmd, num, params):
        num_params = len(params)
        if num_params != num:
            raise argParseException("Incorrect number of parameters for '{}', should be {}, but {} were given".format(cmd, num, num_params))


    # with the default_phonebook option, we have to figure out whether or not to use it
    # (we may be overriding the default by supplying an actual phonebook)
    def create_params_list(cmd, num, params):
        try:
            check_num_params(cmd, num, params)
            return params
        except argParseException as e:
            if default_phonebook is not None:
                params_w_default = params + [default_phonebook]
                check_num_params(cmd, num, params_w_default)
                return params_w_default


    cmd = args[0] # the main command
    cmd_params = args[1:] # parameters to the command

    conn = sqlite3.connect(db_name)
    pbm = PhonebookManager(conn)

    if cmd == 'create':
        check_num_params('create', 1, cmd_params)
        pbm.create_phonebook(cmd_params[0])

    elif cmd == 'lookup':
        params = create_params_list('lookup', 2, cmd_params)
        pbm.lookup_name(params[0], params[1])

    elif cmd == 'add':
        params = create_params_list('add', 3, cmd_params)
        pbm.add_entry(params[0], params[1], params[2])

    elif cmd == 'change':
        params = create_params_list('change', 3, cmd_params)
        pbm.change_entry(params[0], params[1], params[2])

    elif cmd == 'remove':
        params = create_params_list('remove', 2, cmd_params)
        pbm.remove_entry(params[0], params[1])

    elif cmd == 'reverse-lookup':
        params = create_params_list('reverse-lookup', 2, cmd_params)
        pbm.reverse_lookup(params[0], params[1])

    else:
        # try defaulting to 'lookup' command before failing
        try:
            params = create_params_list('lookup', 2, args)
        except Exception as e:
            raise argParseException("Invalid command.")

        pbm.lookup_name(params[0], params[1])



if __name__ == "__main__":
    try:
        # check if there's a config file first
        config = configparser.ConfigParser()
        config.read("phbo.cfg")
        if 'default_phonebook' in config['DEFAULT']:
            exec_command(sys.argv[1:], default_phonebook=config['DEFAULT']['default_phonebook'])
        else:
            exec_command(sys.argv[1:])

    except phonebookException as e:
        print("Error: " + e.value)
    except Exception as e:
        print(e)

