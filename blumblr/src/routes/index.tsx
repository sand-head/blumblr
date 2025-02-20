import type { JSX } from "react";
import type { TuonoProps } from "tuono";
import { Post } from "@core/Post";

interface IndexProps {
  subtitle: string;
  posts: Post[];
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
            <>
              {post.thread?.map((parent) => (
                <section>
                  <header>
                    <img src={parent.author.avatar ?? undefined} width="24px" />
                    {parent.author.displayName ? parent.author.displayName : ""}
                  </header>
                  <div>{parent.text}</div>
                  <div>
                    {parent.embed?.$type === "images" &&
                      parent.embed.images.map((i) => (
                        <img src={i.src} alt={i.altText ?? undefined} />
                      ))}
                  </div>
                </section>
              ))}
              <section>
                <header>
                  <img src={post.author.avatar ?? undefined} width="24px" />
                  {post.author.displayName ? post.author.displayName : ""}
                </header>
                <div>{post.text}</div>
                <div>
                  {post.embed?.$type === "images" &&
                    post.embed.images.map((i) => (
                      <img src={i.src} alt={i.altText ?? undefined} />
                    ))}
                </div>
              </section>
            </>
          );
        })}
      </main>
    </>
  );
}
