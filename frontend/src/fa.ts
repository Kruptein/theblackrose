import { dom, library } from "@fortawesome/fontawesome-svg-core";
import { faFilter } from "@fortawesome/free-solid-svg-icons";

export function loadFontAwesome(): void {
    library.add(faFilter);

    dom.watch();
}
