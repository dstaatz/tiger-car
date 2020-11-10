/* Copyright (C) 2020 Dylan Staatz - All Rights Reserved. */


use rosrust::error::{self, rosxmlrpc, tcpros, naming};

error_chain! {
    links {
        RosRust(error::Error, error::ErrorKind);
        XmlRpc(rosxmlrpc::Error, rosxmlrpc::ErrorKind);
        Tcpros(tcpros::Error, tcpros::ErrorKind);
        Naming(naming::Error, naming::ErrorKind);
    }
}

