# TODO

A todo list is an excellent way to handle passing arguments to a program. On the
flip side, there is also the need to get certain system information. For
example, RAM, HDD/SSD/NVMe space, user level (admin/root versus standard), etc.

Having the capability to access this information allows for the program to run
in a default mode. For example, a specific window size (terminals typically open
in 80x40), requesting more memory has a few more steps for non-root users
(authorization, OS and hardware access, etc).

## Plan

The idea with this one is to start with a simple TODO CLI program. This means
its needs:
    New (new list)
    Add (add item to list)
    Complete (check off item)
    Delete (remove a list)

This will start off simple:

```sh
todo new <list>
todo add <list> <task>
todo done <list> <task number>
todo delete <list>
todo [alone will print task lists]
todo <list> [will print tasks in the list]
```
## Continuation

In line with this repo, this will also be broken out into different functions
after a working version is built. 

From there, this may get a GUI to with it. Like a Kanban board... Maybe...
