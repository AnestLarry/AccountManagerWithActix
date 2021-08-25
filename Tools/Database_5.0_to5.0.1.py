# It is important that add email field.
# 5.0.0 to 5.0.1

import os.path
import sqlite3
import base64
if not os.path.exists("Database.db"):
    print("Database file is not existed!")
else:
    conn = sqlite3.connect("Database.db")
    c = conn.cursor()
    c.execute('PRAGMA foreign_keys = 0;')
    c.execute("CREATE TABLE sqlitestudio_temp_table AS SELECT * FROM Data;")
    c.execute("DROP TABLE Data;")
    c.execute("""CREATE TABLE Data (
   Address  TEXT,
   Account  TEXT,
   Password TEXT,
   Email    TEXT,
   Date     TEXT,
   Text     TEXT
);""")
    c.execute("""INSERT INTO Data (
  Address,
  Account,
  Password,
  Date,
  Text
)
SELECT Address,
     Account,
     Password,
     Date,
     Text
FROM sqlitestudio_temp_table;""")
    c.execute("""DROP TABLE sqlitestudio_temp_table;""")
    c.execute("""PRAGMA foreign_keys = 1;""")
    conn.commit()
    c.execute("update Note set Note2 = '5.0.1' where Note1 = 'Version';")
    conn.commit()
    conn.close()
    del conn, c
    print("Succ")
