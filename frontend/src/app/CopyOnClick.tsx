import { PropsWithChildren } from "react";
import { Button } from "./Button.tsx";
import { useEvent } from "../utils/useEvent.ts";

export interface CopyOnClickProps {
  data: string;
}

export function CopyOnClick(props: PropsWithChildren<CopyOnClickProps>) {
  const { data, children } = props;

  const onClick = useEvent(() => {
    void navigator.clipboard.writeText(data);
  });

  return <Button onClick={onClick}>{children}</Button>;
}
