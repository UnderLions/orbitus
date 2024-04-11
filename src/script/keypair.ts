import { database } from "./database"
import { randomUUID,createDiffieHellman } from "crypto"

type SessionId = string

type Session_T = {
    username: string,
    session_id : SessionId
}


enum SessionResult {
    SessionNotExists = 0,
    SessionExists = 1,
    // SessionTimeOut,
    // SessionSuspended,
    SessionCreated = 1,
    SessionGenFail = 0,
    Ok
}

class SessionDB {
    db : Session_T[] = []
    Session() {

    }

    newSession( username: string ) : SessionResult {
        if (database.checkUser(username as string)) {
            let new_session_id = randomUUID() as SessionId;
            this.db.push( { username: username , session_id: new_session_id } as Session_T );
            console.info("sessionMgr:", "Created for ",username , new_session_id)
            return SessionResult.SessionCreated
        }
        return SessionResult.SessionGenFail
    }

    getSession( username: string ) : SessionResult | string {
        for ( let Session of this.db ) {
            if (Session.username === username) {
                return Session.session_id
            }
        }
        return SessionResult.SessionNotExists
    }
    checkSession( session_id: SessionId ) {
        for ( let Session of this.db ) {
            if (Session.session_id === session_id) {
                return SessionResult.SessionExists
            }
        }
        return SessionResult.SessionNotExists
    }

    getUsername( session_id: SessionId ) : SessionResult | string {
        for ( let Session of this.db ) {
            if (Session.session_id === session_id) {
                return Session.username
            }
        }
        return SessionResult.SessionNotExists
    }
}

const sessionMgr = new SessionDB();

export {
    sessionMgr,
    SessionDB,
    SessionResult,
    type SessionId,
    type Session_T,
}