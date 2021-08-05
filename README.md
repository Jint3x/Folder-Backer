## Introduction

This is a small application, which allows users to specify 2 folders, the first folder is the `main` folder and the second one is a `backup` folder. This tool copies all of the contents from `main` to `backup`, however you can filter [exclude] 
folders. 

<br />

## Arguments

You can either use a config file or pass arguments when calling the executable. There are 3 arguments that can be used:  
  
`main_folder`: Main folder which will be copied  
`backup_folder`: Backup folder which will be a replica of the main one  
`filter_dirs`: A comma separated list of folders which will be ignored 

The `saver.conf` config file includes more details on these arguments. 

<br />

## Usage  

<br />

### With arguments [recommended]
`cargo run -- main_folder="C:/path/to/folder" backup_folder="C:/path/to/backup" filter_dirs="dir1, bad dir, another_bad_dir"`

This will copy C:/path/to/folder to C:/path/to/backup and it will ignore folders: " dir1", "bad dir" and "another_bad_dir". 
Passed arguments overwrite the ones provided in the config. If a wrong argument is passed, an error will be thrown.

<br />

### With config file 


```env
# Config File [saver.conf]  

main_folder=C:/path/to/folder
backup_folder=C:/path/to/backup
filter_dirs=dir1, bad dir, another_bad_dir
``` 

`cargo run`  

It will do the same things as the call with arguments. **The config file needs to be in the same folder as the current working directory**





