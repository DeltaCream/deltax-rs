import React from "react";

export default async function Page() {
    return (
        <main>
            <h1>Videos Page</h1>
            <p>This contains various notable videos displayed as a gallery.</p>
            <video width="320" height="240" controls>
                <source src="movie.mp4" type="video/mp4" />
                <source src="movie.ogg" type="video/ogg" />
                <track
                    src="fgsubtitles_en.vtt"
                    kind="subtitles"
                    srcLang="en"
                    label="English"
                />
                Your browser does not support the video tag.
            </video>
            <audio controls>
                <source src="horse.ogg" type="audio/ogg" />
                <source src="horse.mp3" type="audio/mpeg" />
                Your browser does not support the audio element.
            </audio>
            <iframe
                width="420"
                height="315"
                src="https://www.youtube.com/embed/tgbNymZ7vqY"
            />
        </main>
    );
}
