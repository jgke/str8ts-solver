export interface HintProps {
  hint?: string;
}
export function Hint(props: HintProps) {
  const { hint } = props;
  return (
    hint && (
      <div className="my-2 border p-2 dark:bg-blue-400 dark:text-white">
        <b className="font-bold">Hint: {hint}</b>
      </div>
    )
  );
}
