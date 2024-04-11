import { NextRequest, NextResponse } from "next/server";
// import { NextApiRequest, NextApiResponse } from "next";
import { type DataBase_T } from "@/script/database";
import { database , DataBase, Result } from "@/script/database";

const defaultAction = ( action:string | undefined , defaultVal: string):string => {
    if (action===undefined) {
        return defaultVal
    }
    return action
}

export async function POST(request: NextRequest) {

    let formData = await request.formData()
    let data = {
        name: defaultAction(formData.get("name")?.toString(),"Default"),
        password: defaultAction(formData.get("password")?.toString(),"Default")
    } as DataBase_T;

    console.log(` NEw req : ${data.name} ${data.password}`);

    // database update ================>  //
    if (!database.checkUser(data.name as string)) {
        database.push(data);
        console.info(" <------------ user added ------------------------------->")
    } else {
        console.error("<------------ fail to create user ---------------------->")
        let uri = request.nextUrl.clone();
        uri.pathname = "/login";
        return NextResponse.redirect(uri)
    }
    // <================================= //

    return new NextResponse();
}

export function GET( req: NextRequest){
    for (let data of database.database) {
        console.log(`| NAME: ${data.name}  \t| PASSWD: ${data.password}\t|`);
    }
    return NextResponse.json({ status : "Online" })
}