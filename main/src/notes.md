Using TcpListener we can listen for TCP conneciton on the given address

the bind function works like the new function in that it returns a new instance of TcpListener
it retuens a Result\<T, E\>
which indicates that the binding process can not work 
we use .unwrap() to stop the program if error happen

the incoming method on TcpListener returns an iterator that gives us a sequence of streamss 
