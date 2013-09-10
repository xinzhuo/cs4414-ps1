Title: Problem Set 1 Answers
Author: Xinzhuo Dong
1. Mozilla/5.0 (Macintosh; Intel Mac OS X 10_8_4) AppleWebKit/537.4 (KHTML, like Gecko) Chrome/22.0.1229.94 Safari/537.4

Answer: 
"Mozilla" means my browser "pretend" to be a Mozilla based user agent or Mozilla-compatible and 5.0 is the corresponding Mozilla version.
Macintosh is the OS using and Intel Mac OS X 10_8_4 represents OS version and inted based.
AppleWebKit/537.4: the web kit name and build to present web content and KHTML is the HTML layout developed by KDE project used with web kit.
The following is the browser name and version (chrome) which is based on Safari build 537.1

2.
Answer:
Because we want our visitor counter to be preserved in the server and make increment everytime a request arrived, we have to declare our variable static and mutable. In this particular case, since we are the only users that have access to the site localhost:4414, it doesn't really matter. However, it's reasonable to make concern regarding the modification of a global variable, because when requests arrive concurrently, error and inconsistency of the value of the variable are very likely to appear. Thus, programs like this is unsafe.

3.

