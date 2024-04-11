import { NextResponse } from "next/server";
import { NextRequest } from "next/server";
import { sessionMgr } from "./script/keypair";

export function middleware(request: NextRequest) {
    // console.log("server hello --");
    let url = request.nextUrl.clone();
    if (url.pathname === "/protected") {
        let session = request.cookies.get("session")?.value;
        if (sessionMgr.checkSession(session as string)) {
            console.info("<==================== Access Granted =====================>")
            return NextResponse.next()
        }
        console.info("<==================== Access denied =====================>")
        url.pathname = "/"
        return NextResponse.redirect(url)
    }
    return NextResponse.next()
}