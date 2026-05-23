# FlowTree
Watch how the files in a folder and its subfolders change over time

I am a big fan of the linux commands watch and tree. The first re-runs a command for example every second 
and the second shows a tree of files. Together, a command like `watch -n 1 tree foo` can show how new files
and folders are created, but often the tree becomes too big for the screen and at needs iterative tampering with 
the command to stay focused on the relevant part, where the new files are created.
It would be very nice if the tree could stay focused on the relevant parts automatically. For me this would be a
useful to watch the file activity while debugging

## IMPORTANT NOTE
This is my first rust project, and I am just starting, so the goal may stay elusive for some time.
