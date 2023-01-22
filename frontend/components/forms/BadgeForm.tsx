import React from 'react'
import * as crypto from "crypto";
import {useRouter} from "next/router";

type UserProp = {
    badgeName: string,
    badgeId: number,
    deleteFunction?: Function
}


const BadgeForm = (props: UserProp) => {
    const router = useRouter()

    return (
        // TODO fix text contrast :)
        <div className="badge badge-info gap-2 mr-1" style={{
            borderColor: 'transparent',
            backgroundColor: '#' + crypto
                .createHash('md5')
                .update(props.badgeName)
                .digest('hex')
                .substring(0, 6),
            background: '#' + crypto
                .createHash('md5')
                .update(props.badgeName)
                .digest('hex')
                .substring(0, 6),
        }}>
            <button onClick={() => {
                if (props.deleteFunction != undefined) {
                    props.deleteFunction(props.badgeId)
                    router.reload()
                }
            }}>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                     className="inline-block w-4 h-4 stroke-current">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2"
                          d="M6 18L18 6M6 6l12 12"></path>
                </svg>
            </button>
            <p style={{color: (Number(`0x${crypto
                    .createHash('md5')
                    .update(props.badgeName)
                    .digest('hex')
                    .substring(0, 6)}`) ^ 0xFFFFFF)
                    .toString(16)
                    .substring(1)
                    .toUpperCase()}}>{props.badgeName}</p>
        </div>
    )
}

export default BadgeForm