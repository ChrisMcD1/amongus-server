type RoleProps = { username: string };

export default function RoleAssignment(props: RoleProps) {
    return (
        <div className="background flex h-screen items-center bg-black">
            <div className="flex h-fit">
                <img src="../../public/Pictures/impostertemplate.jpg" />
            </div>
        </div>
    );
}
