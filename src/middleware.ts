import { NextResponse } from "next/server";
import { NextRequest } from "next/server";


export function middleware(request: NextRequest) {
    console.log("server hello --");
    let url = request.nextUrl.clone();
    if (url.pathname !== "/login") {
        url.pathname = "/login"
        return NextResponse.redirect(url)
    }
    return NextResponse.next()
}