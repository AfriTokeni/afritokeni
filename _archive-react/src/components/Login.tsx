import { signIn } from "@junobuild/core";
import { FC } from "react";
import { Button } from "./Button";

export const Login: FC = () => {
  const handleSignIn = async () => {
    // Use id.ai domain only in production, default II for localhost
    const isProduction = window.location.hostname !== 'localhost' && window.location.hostname !== '127.0.0.1';
    
    if (isProduction) {
      // Production: Use id.ai with derivationOrigin
      await signIn({
        internet_identity: {
          options: {
            domain: "id.ai",
            derivationOrigin: "https://afritokeni.com",
          },
        },
      });
    } else {
      // Local development: Use default Internet Identity (no id.ai)
      await signIn({
        internet_identity: {},
      });
    }
  };
  
  return <Button onClick={handleSignIn}>Sign in</Button>;
};
