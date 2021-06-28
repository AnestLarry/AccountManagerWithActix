# Database Version: 5.0
import os.path
import sqlite3
if not os.path.exists("Database.db"):
    conn = sqlite3.connect("Database.db")
    c = conn.cursor()
    c.execute('CREATE TABLE Data ("Address" TEXT,"Account" TEXT,"Password" TEXT,"Date" TEXT,"Text" TEXT);')
    c.execute('CREATE TABLE User (Account CLOB,Password CLOB);')
    c.execute('CREATE TABLE Note (Note1 CLOB,Note2 CLOB);')
    conn.commit()
    c.execute("insert into Note values('Version','5.0');")
    c.execute("insert into User values('admin','admin');")
    c.execute('insert into Data values("dGVzdC5jb20=","dGVzdA==","MTIzNDU2","1970-01-01--00-00-00--Thursday","initial Testing");')
    # dGVzdC5jb20= test.com
    # dGVzdA== test
    # MTIzNDU2 123456
    # MTk3MC0wMS0wMS0tMDAtMDAtMDAtLVRodXJzZGF5 1970-01-01--00-00-00--Thursday
    conn.commit()
    conn.close()
    del conn, c
    print("Succ")
else:
    print("Database file is existed!")
