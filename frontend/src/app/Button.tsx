import { ButtonHTMLAttributes, DetailedHTMLProps, LabelHTMLAttributes } from "react";

export function Button(props: DetailedHTMLProps<ButtonHTMLAttributes<HTMLButtonElement>, HTMLButtonElement>) {
  const { className, children, ...rest } = props;
  return (
    <button
      className={`bg-light-800 disabled:text-light-300 dark:disabled:text-light-300 mr-2 flex-grow-0 cursor-pointer rounded border p-2 font-bold text-black disabled:border-transparent dark:border-blue-400 dark:bg-blue-300 dark:text-white ${className ?? ""}`}
      {...rest}
    >
      {children}
    </button>
  );
}

export function LabelButton(props: DetailedHTMLProps<LabelHTMLAttributes<HTMLLabelElement>, HTMLLabelElement>) {
  const { className, children, ...rest } = props;
  return (
    <label
      className={`bg-light-800 disabled:text-light-300 dark:disabled:text-light-300 mr-2 flex-grow-0 cursor-pointer rounded border font-bold text-black disabled:border-transparent dark:border-blue-400 dark:bg-blue-300 dark:text-white ${className ?? ""}`}
      {...rest}
    >
      {children}
    </label>
  );
}
