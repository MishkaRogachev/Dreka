<script lang="ts">
import Drawer, { Content, Header, Title, Subtitle, Scrim } from '@smui/drawer';
import List, { Item, Text, Separator, Subheader } from '@smui/list';

import Topbar from '$pages/topbar/Topbar.svelte';

import Flight from '$pages/flight/FlightPage.svelte';
import Communication from '$pages/communication/CommunicationPage.svelte'
import About from '$pages/about/AboutPage.svelte';

import fleetIcon from "$assets/svg/fleet.svg?raw";
import connectIcon from "$assets/svg/connect.svg?raw";
import aboutIcon from "$assets/svg/about.svg?raw";

enum Pages {
    Flight = "Flight",
    Communication = "Communication",
    About = "About"
}

const pages = [Pages.Flight, Pages.Communication, Pages.About];
let currentPage: Pages = Pages.Flight;

let open = false;

function setPage(page: Pages) { currentPage = page; open = false }

</script>

<style>
.drawer-icon {
    width: 48px;
}
</style>

<Drawer variant="modal" fixed={false} bind:open>
    <Header>
        <Title>Dreaka</Title>
        <Subtitle>Ground Control</Subtitle>
    </Header>
    <Content>
        <List>
        <Item on:click={() => { setPage(Pages.Flight); }}>
            <div class="drawer-icon">{@html fleetIcon}</div> <Text>Flight</Text>
        </Item>
        <Separator />
        <Subheader tag="h6">Settings</Subheader>
        <Item on:click={() => { setPage(Pages.Communication); }}>
            <div class="drawer-icon">{@html connectIcon}</div> <Text>Communication</Text>
        </Item>
        <Item on:click={() => { setPage(Pages.About); }}>
            <div class="drawer-icon">{@html aboutIcon}</div> <Text>About</Text></Item>
        </List>
    </Content>
</Drawer>

<Scrim fixed={false} />

<div id="app" style="background-color:black">
    <Topbar on:openDrawer={() => { open = true; }}/>
    <!-- Main modes, never suspend -->
    <Flight visible={currentPage === Pages.Flight}/>

    <!-- Aux modes -->
    {#if currentPage === Pages.Communication} <Communication/> {/if}
    {#if currentPage === Pages.About} <About/> {/if}
</div>
