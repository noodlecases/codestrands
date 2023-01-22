import React from 'react'
import BadgeForm from './BadgeForm';

type UserProp = {
    name: string,
    badges: string[],
    buttonCaption: string,
}


const BadgeListForm = (props: UserProp) => {
    return (
        <div>
            <div className='px-8 pb-2'>
                <div className="text-xl text-primary-content font-semibold pb-2">{props.name}</div>
                {props.badges.map((badgeName: string) => <BadgeForm badgeName={badgeName} key={badgeName}></BadgeForm>)}
            </div>
            <div className='px-8 pb-8'>
                <div className="dropdown">
                    <label tabIndex={0} className="btn btn-primary mr-1">{props.buttonCaption}</label>
                    <ul tabIndex={0} className="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52">
                        <li><a>Item 1</a></li>
                        <li><a>Item 2</a></li>
                    </ul>
                </div>
            </div>
        </div>
    )
}

export default BadgeListForm