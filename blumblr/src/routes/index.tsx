import type { JSX } from "react";
import type { TuonoProps } from "tuono";

interface BskyPost {
  author: {
    displayName: string | undefined;
    userName: string;
    avatar: string | undefined;
  };
  text: string;
  replies: number;
  likes: number;
  reposts: number;
}
interface IndexProps {
  subtitle: string;
  posts: BskyPost[];
}

export default function IndexPage({
  data,
  isLoading,
}: TuonoProps<IndexProps>): JSX.Element {
  if (isLoading) {
    return <h1>Loading...</h1>;
  }

  console.log("feed", data?.posts);

  return (
    <>
      <main>
        {data?.posts.map((post) => {
          return (
            <section>
              <header>
                <img src={post.author.avatar} width="24px" />
                {post.author.displayName ? post.author.displayName : ""}
              </header>
              <div>{post.text}</div>
            </section>
          );
        })}
      </main>
    </>
  );
}
