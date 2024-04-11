import type { NextApiRequest } from "next";
import { NextApiResponse } from "next";
import { NextURL } from "next/dist/server/web/next-url";
import { Router } from "next/router";
import { NextRequest, NextResponse } from "next/server";
// import { BodyInit } from "next/server"
import { database } from "@/script/database";
import { sessionMgr, SessionResult } from "@/script/keypair";

export async function POST(request: NextRequest) {
    console.log("server recives auth req")
    // let url = request.nextUrl.clone()
    // url.pathname = "/home"
    // console.log("redir to ", url)

    let formData = await request.formData()
    let data = {
        name: formData.get("name")?.toString() || "Default",
        password: formData.get("password")?.toString() || "Default"
    };

    let res = new NextResponse()
    // console.info("Header ------------------------>")
    // for (let i in res.headers.keys()) {
    //     console.info(i.toString())
    // }
    // console.info("<----------------------- Header")
    if (sessionMgr.newSession(data.name as string)) {
        let session = sessionMgr.getSession(data.name) as string;
        res.cookies.set("session", session);
        res.headers.set("status", "200");
        console.info("<-------------- New session for ",data.name,"--------------------> ")
        return res
    }
    console.error(`Failed init session for ${data.name} ------------------------->`)
    res.headers.set("status", "402")
    return res
    // return NextResponse.redirect(url)
}