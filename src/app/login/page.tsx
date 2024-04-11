'use client';
import { Auth } from "@/components/auth";
import { useState } from "react";


export default function add() {
    var [isAdd , setAdd] = useState(true);

    return <div>
        <div className="flex flex-col items-center w-screen h-screen justify-center">
            <Auth name={isAdd?"Add user":"Login"} to={isAdd?"/auth/add":"/auth/kerbros"}/>
            <p onClick={()=>setAdd(!isAdd)}>
                { isAdd?"login":"signup" }
            </p>
        </div>
    </div>
}