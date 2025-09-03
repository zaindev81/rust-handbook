# practice-1

```sh
cargo run

http :3000
http :3000/hello
http :3000/hello/John
http :3000/hello/query msg==Hi
http :3000/users
http :3000/users/1
http POST :3000/users username=John
http PATCH :3000/users/1 username=new_name
http DELETE :3000/users/1
```