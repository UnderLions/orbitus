import { Fragment } from "react";

export function Auth(data: { name: String, to: string }) {
    return <Fragment>
        <form method="post" action={data.to} className="flex flex-col justify-center items-center">
            <p className=" font-bold m-2 self-start p-2 px-4 bg-black text-white">{data.name}</p>
            <input type="text" name="name" className="p-2 m-2 w-96" />
            <input type="password" name="password" className="p-2 m-2 w-96" />
            <button type="submit" className="font-bold bg-blue-700 text-white p-2 px-4 m-2 hover:shadow-md shadow-blue-400 self-end">Submit</button>
        </form>
    </Fragment>
}