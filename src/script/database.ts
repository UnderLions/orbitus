
interface DataBase_T {
    name : String,
    password : String
};

// let database : DataBase_T[] = [];

enum Result {
    UserNotFound,
    PasswordMissMatch,
    UserExists = 1,
    UserNotExists = 0,
    // UnknownError,
    Ok
}

class DataBase {
    database: DataBase_T[] = new Array();

    DataBase() {
        console.log("database initlized");
    }

    push(data: DataBase_T) {
        console.log(`new user ${data.name} is about to add`);
        this.database.push(data);
    }

    checkUser( username: string) {
        for (let data of this.database) {
            if (username === data.name) {
                return Result.UserExists;
            }
        }
        return Result.UserNotExists
    }

    check(user : DataBase_T) : Result {
        let user_exists = false;
        for (let data of this.database) {
            if (user.name === data.name) {
                if (user.password === user.password) {
                    return Result.Ok;
                }
                return Result.PasswordMissMatch;
            }
        }
        return Result.UserNotFound;
    }
}

const database : DataBase = new DataBase()

export {
    database,
    DataBase,
    type DataBase_T,
    Result
};