import { useAppSelector } from "./hooks";

export default function ErrorMessage() {
    const message = useAppSelector((state) => state.errors.error);
    return (
        <div className="bg-red-800 ">
            <h1 className="text-3xl mx-auto my-auto">{message}</h1>
        </div>
    )
}
