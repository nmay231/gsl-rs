// Inlined and adapted from tauri-plugin-htmx
// https://github.com/roniemartinez/tauri-plugin-htmx/blob/230568905b87a876e6bf842cc84b3b266bc8a5fa/LICENSE
//
// MIT License
//
// Copyright (c) 2023 Ronie Martinez
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

const { invoke } = window.__TAURI__.tauri;
// TODO: Something to look at in the future:
// const { fetch: tauriFetch } = window.__TAURI__.tauri;

const BACKEND_PREFIX = "command:";
const REMOTE_SERVER_PREFIX = "server:";

const backendSend = async function (params) {
    // Make readonly properties writable
    Object.defineProperty(this, "readyState", { writable: true })
    Object.defineProperty(this, "status", { writable: true })
    Object.defineProperty(this, "statusText", { writable: true })
    Object.defineProperty(this, "response", { writable: true })

    // Set response
    const query = new URLSearchParams(params);
    this.response = await invoke(this.command, Object.fromEntries(query));
    this.readyState = XMLHttpRequest.DONE;
    this.status = 200;
    this.statusText = "OK";

    // We only need load event to trigger a XHR response
    this.dispatchEvent(new ProgressEvent("load"));
};

const remoteServerSend = async function (params) {
    Object.defineProperty(this, "readyState", { writable: true })
    Object.defineProperty(this, "status", { writable: true })
    Object.defineProperty(this, "statusText", { writable: true })
    Object.defineProperty(this, "response", { writable: true })

    if (this.verb === "get") {
        this.response = await invoke("remote_server_get", { path: this.path });
    } else {
        const query = new URLSearchParams(params);
        this.response = await invoke(`remote_server_${this.verb}`, { path: this.path, params: Object.fromEntries(query) });
    }
    this.readyState = XMLHttpRequest.DONE;
    this.status = 200;
    this.statusText = "OK";

    this.dispatchEvent(new ProgressEvent("load"));
};

window.addEventListener("DOMContentLoaded", () => {
    document.body.addEventListener('htmx:beforeSend', (event) => {
        console.log("listener attached")
        const path = event.detail.requestConfig.path;
        if (path.startsWith(REMOTE_SERVER_PREFIX)) {
            // Route remote server requests through backend to avoid CORS
            event.detail.xhr.path = path.slice(REMOTE_SERVER_PREFIX.length);
            event.detail.xhr.verb = event.detail.requestConfig.verb.toLowerCase();
            event.detail.xhr.send = remoteServerSend;
        } else if (path.startsWith(BACKEND_PREFIX)) {
            event.detail.xhr.command = path.slice(BACKEND_PREFIX.length);
            event.detail.xhr.send = backendSend;
        }
    });
});
