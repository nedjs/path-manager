# path-manager (pman.exe)
Windows application which manages binaries and library dependencies which live in your path. This is similar to the `alternatives` command in unix (but with differnt syntax). 

## Downloading and building
Builds using `cargo`, a pretty common build method and much the same as any other rust project I would imagine.
```ssh
git clone https://github.com/nedjs/path-manager.git
cd path-manager
cargo build
```


## Contributing
As a standalone tool which probably wont hit many keywords in search engines I dont expect to many people adding to the tool. In addition to being that standalone tool there arent to many big TODOs on this project. Its my first Rust project and first project ive actually put on Github, so just learning the ropes here.

TODOS:
  - Search for "TODO" in project and you'll find my TODO comments
  - Add tests, when I wrote this I was just learning the syntax. Next step is learning testing in rust... it really should be some testing
  - Add functionallity to allow for global configuration alterations at runtime which dont persist. Two I can think of are: specify config location (--cfg) and command directory (--cmd) 


## Using the binary

### Installation
Use `pman help` to get all the help goodies. Before using the executable for things other than help content you will need to set a command directory. To do this run `pman configure -p`, this will bring you through some prompts to setup the configuration.

The configuration is stored in the same folder as the executable and named `.{binary_name}`, so if its `pman.exe` the config will be `.pman`. The format for this isnt any normal file format but is human readable if you want to manually edit the configuration.

### Sample usage (groups)
Here i'll go through steps to setup a standard Java installation. This should be applicable to many different libraries.

Ill be setting this up on my `D:\` where I have already made the folder `D:\bin` and added that folder to my systems `%PATH%` value. This is where `pman.exe` will live and the executables generated from pman will be saved, I recommend having this as its own folder and not sharing it with something else. Also dont be dumb and point it to your System32 folder or something of that nature.


1. Setup the command directory to the same folder as the executable.
      ```
    > pman configure -d .
    Command directory updated to: '.'
    ```
2. Add my Java7 installation as a group. Groups are just a set of links to files in an installation. *Also in case you arent familar with windows multi line commands; the caret `^` is how you enter commands on more than one line.*
      ```
    > pman group java 170 ^
    -add ^
    -dir "D:\lib\java\jdk1.7.0_79\bin" ^
    -link java java.exe ^
    -link javac javac.exe ^
    -link java_home ../
    
    Created new group java/170
    Adding link "java" => "java.exe"
    Adding link "javac" => "javac.exe"
    Adding link "java_home" => "../"
    Setting base directory D:\lib\java\jdk1.7.0_79\bin
    ```
3. Activate the group you just added
    ```
    > pman swap java 170
    Swapping to 170 - D:\lib\java\jdk1.7.0_79\bin
    ```
4. you should now have 3 new files in the command folder:
    ```
     bin/
      java.bat - runs D:\lib\java\jdk1.7.0_79\bin\java.exe %*
      javac.bat - runs D:\lib\java\jdk1.7.0_79\bin\javac.exe %*
      java_home - is a symlink to D:\lib\java\jdk1.7.0_79
    ```
5. At last everything is configured, you can use those 2 java executables from anywhere in your path
    ```
    > java -version
    java version "1.7.0_79"
    Java(TM) SE Runtime Environment (build 1.7.0_79-b15)
    Java HotSpot(TM) 64-Bit Server VM (build 24.79-b02, mixed mode)
    ```
6. You may have noticed the symlink for `java_home` I also set, you can now set your enviroment variable `%JAVA_HOME%` to point to `D:\bin\java_home` and when you swap between java versions the variable will always be valid.

### Sample usage (links)
Not all binarys you want in your class path are part of a versioned thing you want to toggle. You can also create a singular link which points to a exectuable or folder.

```
> pscp -V
'pscp' is not recognized as an internal or external command,
operable program or batch file.

> pman link -l pscp D:\lib\putty\pscp.exe
Link created "pscp" => "D:\\lib\\putty\\pscp.exe"

> pscp -V
pscp: Release 0.67
```

likewise to unlink something you can use the `pman link -u <name>` to remove it
```
> pscp -V
pscp: Release 0.67

> pman link -u pscp 
Link removed "pscp" => "D:\\lib\\putty\\pscp.exe"

> pscp -V
'pscp' is not recognized as an internal or external command,
operable program or batch file.
```

