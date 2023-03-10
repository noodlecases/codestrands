import React from 'react'
import BadgeForm from './BadgeForm';
import {useRouter} from "next/router";


type Badge = {
    id: number,
    name: string,
    toggleFunction?: Function,
}

type UserProp = {
    name: string,
    badges: Badge[],
    allBadges: Badge[],
    buttonCaption: string,
}


const BadgeListForm = (props: UserProp) => {
    const router = useRouter()

    return (
        <div>
            <div className='px-8 pb-2'>
                <div className="text-xl text-primary-content font-semibold pb-2">{props.name}</div>
                {props.badges.map((badge: Badge) => <BadgeForm badgeName={badge.name} key={badge.id}
                                                               badgeId={badge.id}
                                                               deleteFunction={badge.toggleFunction}></BadgeForm>)}
            </div>
            <div className='px-8 pb-8'>
                <div className="dropdown">
                    <label tabIndex={0} className="btn btn-primary mr-1">{props.buttonCaption}</label>
                    <ul tabIndex={0} className="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52">
                        {props.allBadges.map((badge: Badge) => <li>
                            <button onClick={() => {
                                if (badge.toggleFunction != undefined) {
                                    badge.toggleFunction(badge.id)
                                    router.reload()
                                }
                            }}> {badge.name}</button>
                        </li>)}
                    </ul>
                </div>
            </div>
        </div>
    )
}

export default BadgeListForm